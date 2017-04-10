import uuid 

from django.contrib.auth.models import User
from django.db import models
from datetime import timedelta

class Time(models.Model):
    start = models.DateTimeField('Start Time')
    end = models.DateTimeField('End Time', blank=True, null=True)
    lunch = models.IntegerField('Lunch', default=0)

    morning_task = models.TextField(blank=True, null=True)
    afternoon_task = models.TextField(blank=True, null=True)

    user = models.ForeignKey(User, unique=False)

    def __str__(self):
        return 'Timesheet Entry: {}'.format(self.start.strftime('%x'))

    @property
    def hours_worked(self):
        try:
            duration = self.end - self.start - timedelta(minutes=self.lunch)
            return duration.total_seconds() / 3600
        except TypeError:
            return 0


class TimeSlice(models.Model):
    start = models.DateTimeField('Start Date')
    end = models.DateTimeField('End Date')
    user = models.ForeignKey(User, unique=False)

    unique_id = models.UUIDField(default=uuid.uuid4, editable=False, unique=True)
    can_view_tasks = models.BooleanField(default=False)

    def __str__(self):
        return '<{}: start="{}" end="{}" view_tasks={}>'.format(
            self.__class__.__name__,
            self.start.strftime('%x'),
            self.end.strftime('%x'),
            self.can_view_tasks)

    def times(self):
        """
        Get all the `Times` which this slice covers.
        """
        return Time.objects.filter(user=self.user).filter(start__gte=self.start).filter(end__lte=self.end)
