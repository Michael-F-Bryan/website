from django.shortcuts import render, get_object_or_404, redirect
from django.http import HttpResponse
from django.contrib.auth.decorators import login_required
from django.views import View

from .models import Time
from .forms import TimeForm


@login_required
def detail(request, time_id):
    time = get_object_or_404(Time, pk=time_id)
    return render(request, 'times/detail.html', {'time': time})


@login_required
def list_all(request):
    times = Time.objects.all()
    return render(request, 'times/list_all.html', {'times': times})


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
