# -*- coding: utf-8 -*-
# Generated by Django 1.11 on 2017-04-30 17:04
from __future__ import unicode_literals

from django.conf import settings
from django.db import migrations, models
import django.db.models.deletion


class Migration(migrations.Migration):

    dependencies = [
        migrations.swappable_dependency(settings.AUTH_USER_MODEL),
        ('times', '0006_timeslice'),
    ]

    operations = [
        migrations.AddField(
            model_name='timeslice',
            name='created_by',
            field=models.ForeignKey(blank=True, editable=False, null=True, on_delete=django.db.models.deletion.CASCADE, related_name='timeslice_created_by', to=settings.AUTH_USER_MODEL),
        ),
    ]
