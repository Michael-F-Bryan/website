<template>
    <b-container>
        <b-row align-h="center">
            <h1 class="my-md-3">Admin Dashboard</h1>
        </b-row>

        <b-row align-h="end">
            <b-col lg="4" md="auto">
                <b-button-group>
                    <b-button>Create User</b-button>
                    <b-button>Backup</b-button>
                    <b-button>Restore</b-button>
                </b-button-group>
            </b-col>
        </b-row>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component({})
export default class AdminDashboard extends Vue { }
</script>