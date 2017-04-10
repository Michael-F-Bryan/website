import datetime
import csv
from io import StringIO
from collections import OrderedDict

from django.shortcuts import render, get_object_or_404, redirect
from django.http import HttpResponse
from django.contrib.auth.decorators import login_required
from django.views import View
from django.views.decorators.cache import never_cache
from rest_framework import viewsets

from .models import Time
from .forms import TimeForm
from .serializers import TimeSerializer


@login_required
def detail(request, time_id):
    time = get_object_or_404(Time, pk=time_id)
    return render(request, 'times/detail.html', {'time': time})


@login_required
def list_all(request):
    times = Time.objects.all()

    summary = OrderedDict()

    summary['Total Days'] = len(times)
    summary['Total Hours'] =  sum(t.hours_worked for t in times)
    if summary['Total Hours'] > 0:
        summary['Average Work Day'] = summary['Total Hours']/summary['Total Days']
    else:
        summary['Average Work Day'] = 0

    context = {
        'times': times, 
        'summary': summary
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
            time = form.save()
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


