from rest_framework import serializers
from .models import Time

class TimeSerializer(serializers.HyperlinkedModelSerializer):

    class Meta:
        model = Time
        fields = ('start', 'end', 'lunch', 'morning_task', 
                  'afternoon_task')
