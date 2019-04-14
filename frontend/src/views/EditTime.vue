<template>
    <b-container>
        <b-row align-h="center" class="my-md-3">
            <h1>{{title}}</h1>
        </b-row>

        <b-form @submit="submit">
            <b-form-group label="Start" label-for="start" label-cols="2">
                <b-form-input type="datetime-local" name="start" id="start" v-model="startTime"/>
            </b-form-group>

            <b-form-group label="End" label-for="end" label-cols="2">
                <b-form-input type="datetime-local" id="end" name="end" v-model="endTime"/>
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

            <b-row align-h="center">
                <b-form-group>
                    <b-button type="submit">Create</b-button>
                </b-form-group>
            </b-row>
        </b-form>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Entry from '@/client/Entry';
import moment from 'moment';

@Component({})
export default class EditTime extends Vue {
    public start = moment();
    public end = moment().add(8, 'hours');
    public breaks: number = 0;
    public description: string = '';
    public id: string = '';
    public isNew: boolean = true;

    public mounted() {
        const { entryID } = this.$route.params;

        if (entryID) {
            const entry = this.$store.state.times[entryID];
            this.start = entry.start;
            this.end = entry.end;
            this.breaks = entry.breaks.asMinutes();
            this.description = entry.description;
            this.id = entry.id;
            this.isNew = false;
        }
    }

    public get startTime(): string {
        return this.start.format('YYYY-MM-DDThh:mm');
    }

    public set startTime(value: string) {
        this.start = moment(value);
    }

    public get endTime(): string {
        return this.end.format('YYYY-MM-DDThh:mm');
    }

    public set endTime(value: string) {
        this.end = moment(value);
    }

    public submit(e: Event) {
        e.preventDefault();

        const arg = {
            start: this.start,
            end: this.end,
            breaks: this.breaks,
            description: this.description,
            id: this.id,
        };

        this.$store.dispatch('updateTime', arg)
            .then((createdEntry: Entry) => {
                this.$router.push({ name: 'view-time', params: { id: createdEntry.id } });
            }, alert);
    }

    get title(): string {
        return this.isNew ? 'Create Entry' : 'Edit Entry';
    }
}
</script>