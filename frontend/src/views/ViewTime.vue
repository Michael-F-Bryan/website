<template>
    <b-container>
        <b-row>
            <h1>View Time ({{entry.id}})</h1>
        </b-row>
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