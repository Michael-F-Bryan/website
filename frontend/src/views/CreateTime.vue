<template>
    <b-container>
        <b-row>
            <h1>Create Timesheet Entry</h1>
        </b-row>

        <b-form @submit="submit">
            <b-form-group label="Start" label-for="start" label-cols="2">
                <b-form-input type="datetime-local" name="start" id="start" v-model="start"/>
            </b-form-group>
            <b-form-group label="End" label-for="end" label-cols="2">
                <b-form-input type="datetime-local" id="end" name="end" v-model="end"/>
            </b-form-group>
            <b-form-group label="Breaks" label-for="breaks" label-cols="2">
                <b-form-input type="number" min="0" id="breaks" name="breaks" v-model="breaks"/>
            </b-form-group>

            <b-form-group label="Description" label-for="description">
                <b-textarea
                    v-model="description"
                    id="description"
                    name="description"
                    rows="4"
                    autocomplete
                    trim
                    placeholder="Tell me about your day..."
                />
            </b-form-group>

            <b-form-group>
                <b-button type="submit">Create</b-button>
            </b-form-group>
        </b-form>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Entry from '@/client/Entry';
import moment from 'moment';

@Component({})
export default class CreateTime extends Vue {
    public start: string = moment().format('YYYY-MM-DDThh:mm');
    public end: string = moment().add(8, 'hours').format('YYYY-MM-DDThh:mm');
    public breaks: number = 0;
    public description: string = '';

    public submit(e: Event) {
        e.preventDefault();

        const arg = {
            start: this.start,
            end: this.end,
            breaks: this.breaks,
            description: this.description,
        };

        this.$store.dispatch('createTime', arg)
            .then((createdEntry: Entry) => {
                this.$router.push({ name: 'view-time', params: { id: createdEntry.id } });
            }, alert);
    }
}
</script>