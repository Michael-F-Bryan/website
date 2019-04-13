<template>
    <b-container>
        <h1>View Timesheet</h1>

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

    get times(): Entry[] {
        return this.$store.state.times;
    }
}
</script>