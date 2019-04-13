<template>
    <b-navbar toggleable="md" type="dark" variant="info">
        <b-navbar-brand :to="{name: 'home'}">Michael-F-Bryan</b-navbar-brand>

        <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

        <b-collapse id="nav-collapse" is-nav>
            <b-navbar-nav>
                <b-nav-item :to="{name: 'resume'}">Resume</b-nav-item>
                <b-nav-item :to="{name: 'portfolio'}">Portfolio</b-nav-item>
                <b-nav-item :to="{name: 'view-times'}" v-if="user.isLoggedIn">Times</b-nav-item>
                <b-nav-item :to="{name: 'admin-dashboard'}" v-if="user.isAdmin">Admin Dashboard</b-nav-item>
            </b-navbar-nav>

            <b-navbar-nav class="ml-auto">
                <b-nav-item @click="logout" v-if="user.isLoggedIn">Logout ({{user.username}})</b-nav-item>
                <b-nav-item :to="{name: 'login'}" v-else>Login</b-nav-item>
            </b-navbar-nav>
        </b-collapse>
    </b-navbar>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import User from '@/client/User';

@Component({})
export default class Header extends Vue {
    public logout() {
        this.$store.dispatch('logout')
            .then(() => this.$router.push({ name: 'home' }));
    }

    get user(): User {
        return this.$store.state.currentUser;
    }
}
</script>