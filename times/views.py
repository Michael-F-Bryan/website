import datetime
import csv
from io import StringIO
from collections import OrderedDict

from django.shortcuts import render, get_object_or_404, redirect
from django.http import HttpResponse, Http404, HttpResponseForbidden
from django.contrib.auth.decorators import login_required
from django.views import View
from django.views.decorators.cache import never_cache
from rest_framework import viewsets

from .models import Time, TimeSlice
from .forms import TimeForm, TimeSliceForm
from .serializers import TimeSerializer


def summarize(times):
    """
    Iterates through a list of `Times` and summarizes them.
    """
    summary = OrderedDict()

    summary['Total Days'] = len(times)
    summary['Total Hours'] =  sum(t.hours_worked for t in times)

    if summary['Total Hours'] > 0:
        summary['Average Work Day'] = summary['Total Hours']/summary['Total Days']
    else:
        summary['Average Work Day'] = 0

    return summary


@login_required
def detail(request, time_id):
    time = get_object_or_404(Time, pk=time_id)
    if time.user != request.user and not request.is_superuser:
        return HttpResponseForbidden()

    return render(request, 'times/detail.html', {'time': time})


@login_required
def list_all(request):
    times = Time.objects.filter(user=request.user)

    context = {
        'times': times, 
        'summary': summarize(times),
    }
    return render(request, 'times/list_all.html', context)


class TimeEdit(View):
    template_name =  'times/edit.html'

    def get(self, request, time_id):
        time = get_object_or_404(Time, pk=time_id)
        form = TimeForm(instance=time)
        return render(request, self.template_name, {'form': form})

    def post(self, request, time_id):
        time = get_object_or_404(Time, pk=time_id)
        form = TimeForm(request.POST or None, instance=time)

        if form.is_valid():
            form.save()
            return redirect('times:list_all')

        return render(request, self.template_name, {'form': form})

class NewTime(View):
    template_name =  'times/edit.html'

    def get(self, request):
        initial_data = {
            'start': datetime.datetime.now(),
            'end': datetime.datetime.now() + datetime.timedelta(hours=8),
            'lunch': 0,
        }

        form = TimeForm(initial_data)
        return render(request, 'times/new.html', {'form': form})

    def post(self, request):
        form = TimeForm(request.POST)

        if form.is_valid():
            time = form.save(commit=False)
            time.user = request.user
            time.save()

            return redirect('times:detail', time_id=time.id)

        return render(request, self.template_name, {'form': form})


@login_required
def delete(request, time_id):
    time = get_object_or_404(Time, pk=time_id)
    time.delete()
    return redirect('times:list_all')


def gen_filename(date):
    return 'timesheet_{}.csv'.format(date.strftime('%Y-%m-%d'))


@login_required
@never_cache
def download_as_csv(request):
    response = HttpResponse(content_type='text/csv')
    filename = gen_filename(datetime.datetime.now())
    response['Content-Disposition'] = 'attachment; filename="{}"'.format(filename)

    writer = csv.writer(response, dialect='excel')
    writer.writerow(['Start', 'End', 'Hours Worked'])

    headings = ['start', 'end', 'hours_worked']
    for time in Time.objects.all():
        values = [time.__getattribute__(heading) for heading in headings]
        writer.writerow(values)

    return response


class TimeViewSet(viewsets.ModelViewSet):
    """
    API endpoint which allows timesheet entries to be viewed or edited.
    """

    queryset = Time.objects.all()
    serializer_class = TimeSerializer


class CreateTimeSlice(View):
    template_name =  'times/new_time_slice.html'

    def get(self, request):
        now = datetime.datetime.now()
        buffer = datetime.timedelta(hours=10)
        initial_data = {'start': now - buffer, 
                        'end': now + buffer, 
                        'user': request.user}

        form = TimeSliceForm(initial=initial_data)
        return render(request, self.template_name, {'form': form})

    def post(self, request):
        form = TimeSliceForm(request.POST)

        if form.is_valid():
            time_slice = form.save()
            return redirect('times:slice', hash=time_slice.unique_id.hex)

        return render(request, self.template_name, {'form': form})


def slice(request, hash):
    time_slice = get_object_or_404(TimeSlice, unique_id=hash)
    summary = summarize(time_slice.times())
    context = {
        'slice': time_slice,
        'summary': summary,
        'user': time_slice.user,
    }

    return render(request, 'times/time_slice.html', context)


@login_required
def all_time_slices(request):
    slices = TimeSlice.objects.filter(user=request.user)

    context = {
        'slices': slices, 
    }
    return render(request, 'times/all_time_slices.html', context)
