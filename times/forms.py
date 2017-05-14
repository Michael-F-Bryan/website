from django.forms import ModelForm
import emoji

from .models import Time, TimeSlice


class BootstrapModelForm(ModelForm):
    def __init__(self, *args, **kwargs):
        super(BootstrapModelForm, self).__init__(*args, **kwargs)
        for field in iter(self.fields):
            self.fields[field].widget.attrs.update({
                'class': 'form-control'
            })


class TimeForm(BootstrapModelForm):

    class Meta:
        model = Time
        fields = ('start', 'end', 'lunch', 'morning_task', 'afternoon_task')


class TimeSliceForm(BootstrapModelForm):

    class Meta:
        model = TimeSlice 
        fields = ('start', 'end', 'user')
