# -*- coding: utf-8 -*-
# Generated by Django 1.11 on 2017-04-07 07:47
from __future__ import unicode_literals

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('times', '0002_auto_20170407_0732'),
    ]

    operations = [
        migrations.AddField(
            model_name='time',
            name='lunch',
            field=models.IntegerField(default=0, verbose_name='Lunch'),
        ),
    ]