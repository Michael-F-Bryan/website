<template>
    <b-container>
        <b-row class="justify-content-md-center">
            <h1>View Timesheet</h1>
        </b-row>

        <b-row align-h="end">
            <b-col lg="4" md="auto">
                <b-button-group>
                    <b-button :to="{name: 'new-time'}">New</b-button>
                    <b-button @click="download">Download</b-button>
                    <b-button>Share</b-button>
                </b-button-group>
            </b-col>
        </b-row>

        <br>

        <b-row class="justify-content-md-center">
            <b-col md="8">
                <table class="table">
                    <thead>
                        <tr>
                            <td>#</td>
                            <td>Date</td>
                            <td>Start</td>
                            <td>End</td>
                            <td>Time Worked</td>
                            <td></td>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="(entry, id, ix) in times" :key="id">
                            <td>{{ix+1}}</td>
                            <td>
                                <b-link
                                    :to="{name: 'view-time', params: {id}}"
                                >{{entry.start.format('dddd MMMM Do, YYYY')}}</b-link>
                            </td>
                            <td>{{entry.start.format('LT')}}</td>
                            <td>{{entry.end.format('LT')}}</td>
                            <td>{{entry.timeWorked.humanize()}}</td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </b-col>
        </b-row>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Entry from '@/client/Entry';

@Component({})
export default class ViewTimes extends Vue {
    public mounted() {
        this.$store.dispatch('fetchTimes');
    }

    public download() {
        const lines = ['Start,End,Time Worked'];

        Object.values(this.times)
            .sort((l: Entry, r: Entry) => l.start.diff(r.start))
            .forEach((entry: Entry) => {
                const end = entry.end !== undefined ? entry.end.toISOString() : '';
                const line = entry.start.toISOString()
                    + ',' + end
                    + ',' + entry.timeWorked.asHours().toString();
                lines.push(line);
            });

        this.downloadContent('times.csv', lines.join('\n'), 'text/csv');
    }

    private downloadContent(filename: string, body: string, mimeType?: string) {
        // taken straight from: https://stackoverflow.com/a/35251739
        mimeType = mimeType || 'text/plain';

        const blob = new Blob([body], { type: mimeType });

        const dlink = document.createElement('a');
        dlink.download = filename;
        dlink.href = window.URL.createObjectURL(blob);
        dlink.onclick = (e) => {
            setTimeout(() => window.URL.revokeObjectURL(dlink.href), 1500);
        };

        dlink.click();
        dlink.remove();
    }

    get times(): Entry[] {
        return this.$store.state.times;
    }
}
</script>