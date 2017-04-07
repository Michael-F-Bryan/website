from django.conf.urls import url
from . import views

urlpatterns = [
    url(r'^(?P<time_id>\d+)$', views.detail, name='detail'),
    url(r'^$', views.list_all, name='list_all'),
]
