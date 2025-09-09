<template>
    <div style="margin-bottom: 10px">
        <h1>Legal</h1>

        <h2>CCP</h2>
        <span>
            EVE Online and the EVE logo are the registered trademarks of CCP hf.
            All rights are reserved worldwide. All other trademarks are the
            property of their respective owners. EVE Online, the EVE logo, EVE
            and all associated logos and designs are the intellectual property
            of CCP hf. All artwork, screenshots, characters, vehicles,
            storylines, world facts or other recognizable features of the
            intellectual property relating to these trademarks are likewise the
            intellectual property of CCP hf. CCP hf. has granted permission to
            starfoundry.space to use EVE Online and all associated logos and
            designs for promotional and information purposes on its website but
            does not endorse, and is not in any way affiliated with,
            starfoundry.space. CCP is in no way responsible for the content on
            or functioning of this website, nor can it be liable for any damage
            arising from the use of this website.
        </span>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';

@Component
class Legal extends Vue {}

export default toNative(Legal);
</script>
