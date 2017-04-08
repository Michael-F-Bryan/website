from django.db import models
from datetime import timedelta

class Time(models.Model):
    start = models.DateTimeField('Start Time')
    end = models.DateTimeField('End Time', blank=True, null=True)
    lunch = models.IntegerField('Lunch', default=0)

    morning_task = models.TextField(blank=True, null=True)
    afternoon_task = models.TextField(blank=True, null=True)

    def __str__(self):
        return 'Timesheet Entry: {}'.format(self.start.strftime('%x'))

    def hours_worked(self):
        duration = self.end - self.start - timedelta(minutes=self.lunch)
        return duration.total_seconds() / 3600


