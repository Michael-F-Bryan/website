from django.conf.urls import url
from django.contrib.auth.decorators import login_required

from . import views

urlpatterns = [
    url(r'^new$', login_required(views.NewTime.as_view()), name='new'),
    url(r'^(?P<time_id>\d+)/delete$', views.delete, name='delete'),
    url(r'^(?P<time_id>\d+)/edit$', login_required(views.TimeEdit.as_view()), name='edit'),
    url(r'^(?P<time_id>\d+)$', views.detail, name='detail'),
    url(r'^time-slice/(?P<hash>[\d\w]+)$', views.slice, name='slice'),
    url(r'^time-slice/new$', login_required(views.CreateTimeSlice.as_view()), name='new_time_slice'),
    url(r'^time-slice$', views.all_time_slices, name='time_slices'),
    url(r'^download$', views.download_as_csv, name='download'),
    url(r'^$', views.list_all, name='list_all'),
]
