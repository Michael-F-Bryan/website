<template>
    <b-container>
        <b-row align-h="center">
            <h1 class="my-md-3">View Time ({{entry.start.format('dddd MMMM Do, YYYY')}})</h1>
        </b-row>

        <b-row align-h="center" class="my-md-1">
            <b-button-group>
                <b-button :to="{name: 'edit-time', params: {entryID: entry.id}}">Edit</b-button>
                <b-button>Delete</b-button>
            </b-button-group>
        </b-row>

        <b-card title="Summary">
            <dl class="row">
                <dt class="col-md-3">Start</dt>
                <dd
                    class="col-md-9"
                    :title="entry.start.format('LLLL')"
                >{{entry.start.format('LT')}}</dd>
                <dt class="col-md-3">End</dt>
                <dd
                    class="col-md-9"
                    :title="entry.end ? entry.end.format('LLLL') : ''"
                >{{entry.end ? entry.end.format('LT') : '-'}}</dd>
                <dt class="col-md-3">Breaks</dt>
                <dd
                    class="col-md-9"
                >{{entry.breaks.asMinutes() > 1 ? entry.breaks.humanize() : '-'}}</dd>
                <dt class="col-md-3">Time Worked</dt>
                <dd class="col-md-9">{{Math.round(entry.timeWorked.asHours()*10)/10}} hrs</dd>
            </dl>
        </b-card>

        <b-row class="mt-md-4" v-if="entry.description">
            <h3>Description</h3>
        </b-row>
        <b-row v-if="entry.description">
            <div v-html="description"></div>
        </b-row>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Entry from '@/client/Entry';
import marked from 'marked';

@Component({})
export default class ViewTime extends Vue {
    public entry: Entry = new Entry('', new Date());

    public mounted() {
        const id = this.$route.params.id;
        this.entry = this.$store.state.times[id];
    }

    get description(): string {
        return marked(this.entry.description);
    }
}
</script>
