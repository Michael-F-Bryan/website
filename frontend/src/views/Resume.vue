<template>
    <b-container>
        <p class="my-md-3">
            <small class="text-muted">
                <a href="./resume.pdf">click here to download the PDF.</a>
            </small>
        </p>

        <object
            ref="viewer"
            :data="`./resume.pdf`"
            type="application/pdf"
            width="100%"
            height="750px"
        >
            <p>
                It appears you don't have a PDF plugin for this browser.
                No biggie... you can
                <a
                    hread="./resume.pdf"
                >
                    click here to
                    download the PDF file.
                </a>
            </p>
        </object>
    </b-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component({})
export default class Resume extends Vue {
    public mounted() {
        window.addEventListener('resize', this.updateDimensions);

        this.updateDimensions();
    }

    public beforeDestroy() {
        window.removeEventListener('resize', this.updateDimensions);
    }

    private updateDimensions() {
        const viewer = this.$refs.viewer;

        if (viewer instanceof HTMLObjectElement) {
            const remainingWindowHeight = window.innerHeight - viewer.offsetTop - 20;
            viewer.height = Math.max(remainingWindowHeight, 300).toString();
        }
    }
}
</script>