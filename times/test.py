from django.contrib.auth.models import User
from django.test import TestCase
import datetime
import uuid
import pytz

from .models import TimeSlice, Time
from .forms import TimeSliceForm


class TimeSliceTest(TestCase):
    def setUp(self):
        self.user = User(email='michael@example.com', username='michael')
        self.user.save()

        start = datetime.datetime.now(pytz.utc)
        diff = datetime.timedelta(hours=8)

        self.times = [
            Time(start=start, end=start+diff, user=self.user),
            Time(start=start+2*diff, end=start+3*diff, user=self.user),
        ]

        for time in self.times:
            time.save()

        # Make another user with an identical time
        fred = User(email='fred@gmail.com', username='fred')
        fred.save()
        fred_time = Time(start=self.times[0].start, end=self.times[0].end, user=fred)
        fred_time.save()

    def test_create_timeslice_with_form(self):
        start = datetime.datetime.now(pytz.utc)
        end = start + datetime.timedelta(days=3)

        form = TimeSliceForm({
            'start': start,
            'end': end,
            'user': self.user.id,
            'can_view_tasks': True,
        })

        self.assertTrue(form.is_valid())
        print(form.cleaned_data)

        slice = form.save()

        print(slice.start, start)
        self.assertEqual(slice.start, start)
        self.assertEqual(slice.end, end)
        self.assertEqual(slice.user, self.user)
        self.assertEqual(slice.can_view_tasks, True)

    def test_get_times_pointed_to_by_slice(self):
        start = self.times[0].start - datetime.timedelta(hours=1)
        end = self.times[0].end + datetime.timedelta(hours=1)

        slice = TimeSlice(start=start, end=end, user=self.user, can_view_tasks=True)

        got = list(slice.times())
        should_be = self.times[:1]

        self.assertEqual(got, should_be)
