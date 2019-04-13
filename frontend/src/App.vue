<template>
    <div>
        <Header/>
        <router-view/>
    </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Header from '@/components/Header.vue';

interface Breadcrumb {
    text: string;
    href: string;
}

@Component({ components: { Header } })
export default class App extends Vue { }
</script>

