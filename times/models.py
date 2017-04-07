from django.db import models

class Time(models.Model):
    start = models.DateTimeField('Start Time')
    end = models.DateTimeField('End Time', null=True)
    morning_task = models.TextField(null=True)
    afternoon_task = models.TextField(null=True)

    def __str__(self):
        return 'Timesheet Entry: {}'.format(self.start.strftime('%x'))
