from django.conf.urls import url
from django.contrib.auth.decorators import login_required

from . import views

urlpatterns = [
    url(r'^(?P<time_id>\d+)$', views.detail, name='detail'),
    url(r'^new$', login_required(views.NewTime.as_view()), name='new'),
    url(r'^edit/(?P<time_id>\d+)$', login_required(views.TimeEdit.as_view()), name='edit'),
    url(r'^$', views.list_all, name='list_all'),
]
