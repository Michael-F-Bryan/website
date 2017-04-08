from django.forms import ModelForm

from .models import Time


class BootstrapModelForm(ModelForm):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        for field in iter(self.fields):
            self.fields[field].widget.attrs.update({
                'class': 'form-control'
            })


class TimeForm(BootstrapModelForm):

    class Meta:
        model = Time
        fields = ('start', 'end', 'lunch', 'morning_task', 'afternoon_task')


