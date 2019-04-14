<template>
    <b-container>
        <b-row align-h="center">
            <h1 class="my-md-3">View Time ({{entry.start.format('dddd MMMM Do, YYYY')}})</h1>
        </b-row>

        <b-card title="Summary">
            <dl class="row">
                <dt class="col-md-3">Start</dt>
                <dd
                    class="col-md-9"
                    :title="entry.start.format('LLLL')"
                >{{entry.start.format('LT')}}</dd>
                <dt class="col-md-3">End</dt>
                <dd class="col-md-9" :title="entry.end.format('LLLL')">{{entry.end.format('LT')}}</dd>
                <dt class="col-md-3">Breaks</dt>
                <dd
                    class="col-md-9"
                >{{entry.breaks.asMinutes() > 1 ? entry.breaks.humanize() : '-'}}</dd>
                <dt class="col-md-3">Time Worked</dt>
                <dd class="col-md-9">{{Math.round(entry.timeWorked.asHours()*10)/10}} hrs</dd>
            </dl>
        </b-card>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Entry from '@/client/Entry';

@Component({})
export default class ViewTime extends Vue {
    public entry: Entry = new Entry('', new Date());

    public mounted() {
        const id = this.$route.params.id;
        this.entry = this.$store.state.times[id];
    }
}
</script>
