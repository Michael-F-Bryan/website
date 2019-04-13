<template>
    <b-container>
        <h1>Login</h1>

        <b-row class="justify-content-md-center">
            <b-col md="6">
                <b-form @submit="onSubmit" autocomplete="on">
                    <b-form-group label="Username" label-for="name" label-cols-md="2">
                        <b-form-input type="text" v-model="username" name="name"/>
                    </b-form-group>

                    <b-form-group label="Password" label-for="password" label-cols-md="2">
                        <b-form-input type="password" v-model="password" name="password"/>
                    </b-form-group>

                    <b-form-group>
                        <b-button type="submit" :disabled="!valid">Login</b-button>
                    </b-form-group>
                </b-form>
            </b-col>
        </b-row>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component({})
export default class Login extends Vue {
    public username: string = '';
    public password: string = '';

    public onSubmit(e: Event) {
        e.preventDefault();

        this.$store.dispatch('login', { username: this.username, password: this.password })
            .then(() => {
                this.$router.push({ name: 'home' });
            }
                , alert);
    }

    get valid(): boolean {
        return this.username.length > 0 && this.password.length > 0;
    }
}
</script>