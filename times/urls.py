from django.conf.urls import url
from django.contrib.auth.decorators import login_required

from . import views

urlpatterns = [
    url(r'^(?P<time_id>\d+)$', views.detail, name='detail'),
    url(r'^delete/(?P<time_id>\d+)$', views.delete, name='delete'),
    url(r'^new$', login_required(views.NewTime.as_view()), name='new'),
    url(r'^time-slice/(?P<hash>[\d\w]+)$', views.slice, name='slice'),
    url(r'^time-slice/new$', login_required(views.CreateTimeSlice.as_view()), name='new_time_slice'),
    url(r'^time-slice$', views.all_time_slices, name='time_slices'),
    url(r'^download$', views.download_as_csv, name='download'),
    url(r'^edit/(?P<time_id>\d+)$', login_required(views.TimeEdit.as_view()), name='edit'),
    url(r'^$', views.list_all, name='list_all'),
]
