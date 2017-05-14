import datetime
import csv
from io import StringIO
from collections import OrderedDict
import logging

from django.shortcuts import render, get_object_or_404, redirect
from django.http import HttpResponse, Http404, HttpResponseForbidden
from django.contrib.auth.decorators import login_required
from django.views import View
from django.views.decorators.cache import never_cache
from rest_framework import viewsets
from ipware.ip import get_real_ip

from .models import Time, TimeSlice
from .forms import TimeForm, TimeSliceForm
from .serializers import TimeSerializer

logger = logging.getLogger(__name__)

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
        peer_ip = get_real_ip(request)
        logger.warning("Someone tried to access a timesheet entry they don't "
                        "have permissions for (time_id=%d, ip=%s)", 
                        time_id, 
                        peer_ip or "UNKNOWN")
        return HttpResponseForbidden()

    logger.info("User viewed timesheet entry (user=%s, time_id=%s)",
                 request.user, time_id)

    return render(request, 'times/detail.html', {'time': time})


@login_required
def list_all(request):
    times = Time.objects.filter(user=request.user)
    logger.info("User viewed all timesheet entries (user=%s)", request.user)

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
            time = form.save(commit=False)
            time.emojize()
            time.save()

            logger.info("User updated timesheet (user=%s, time_id=%d)", 
                         request.user, time_id)
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
            time.emojize()
            time.user = request.user
            time.save()

            logger.info("Timesheet entry created by %s (time_id=%d)",
                         request.user,
                         time.id)
            return redirect('times:detail', time_id=time.id)

        return render(request, self.template_name, {'form': form})


@login_required
def delete(request, time_id):
    time = get_object_or_404(Time, pk=time_id)
    logger.info("Timesheet entry deleted by %s (time_id=%d)",
                    request.user,
                    time.id)
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
            logger.info("Timeslice created by %s (uuid=%s)", request.user,
                         time_slice.unique_id.hex)
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

    logger.info("Time slice viewed by %s (uuid=%s)", 
                 request.user or get_real_ip(request),
                 time_slice.unique_id.hex)

    return render(request, 'times/time_slice.html', context)


@login_required
def all_time_slices(request):
    slices = TimeSlice.objects.filter(user=request.user)

    context = {
        'slices': slices, 
    }
    return render(request, 'times/all_time_slices.html', context)


class TimeSliceEdit(View):
    template_name =  'times/edit_time_slice.html'

    def get(self, request, hash):
        ts = get_object_or_404(TimeSlice, unique_id=hash)
        form = TimeSliceForm(instance=ts)
        return render(request, self.template_name, {'form': form})

    def post(self, request, hash):
        ts = get_object_or_404(TimeSlice, unique_id=hash)
        form = TimeSliceForm(request.POST or None, instance=ts)

        if form.is_valid():
            form.save()
            logger.info("Time slice updated by %s (uuid=%s)",
                         request.user,
                         ts.unique_id.hex)
            return redirect('times:time_slices')

        return render(request, self.template_name, {'form': form})

@login_required
def delete_time_slice(request, hash):
    ts = get_object_or_404(TimeSlice, unique_id=hash)
    logger.info("Time slice deleted by %s (uuid=%s)",
                    request.user,
                    ts.unique_id.hex)
    ts.delete()
    return redirect('times:time_slices')

