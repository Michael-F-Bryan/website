from django.shortcuts import render, get_object_or_404
from django.http import HttpResponse
from django.contrib.auth.decorators import login_required


from .models import Time

@login_required
def detail(request, time_id):
    time = get_object_or_404(Time, pk=time_id)
    return render(request, 'times/detail.html', {'time': time})

@login_required
def list_all(request):
    times = Time.objects.all()
    return render(request, 'times/list_all.html', {'times': times})
