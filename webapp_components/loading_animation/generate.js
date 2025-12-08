const fs = require('node:fs');
const map_data = require('./map.json')

let xMin = 0;
let xMax = 0;
let zMin = 0;
let zMax = 0;

const timings = {};
map_data
    .forEach(x => {
        //const start = Math.floor(Math.random() * 4);
        const start = Math.random() * 4;
        timings[x.region_id] = start;
    });

map_data
    .forEach(x => {
        xMin = Math.min(xMin, x.position.x);
        xMax = Math.max(xMax, x.position.x);
        zMin = Math.min(zMin, x.position.z);
        zMax = Math.max(zMax, x.position.z);
    });

const IMAGE_SIZE = 512;
const MAP_SIZE = Math.max((xMax - xMin), (zMax - zMin));

const animation = (region_id) => {
    return `<animate
        attributeType="xml"
        attributeName="fill-opacity"
        begin="${timings[region_id]}s"
        from="1"
        to="0.25"
        values="1;0.75;0.5;0.25;0.5;0.75"
        dur="3s"
        repeatCount="indefinite"
    />`
}

const points = map_data
    .map(x => {
        const xPos = (x.position.x - xMin) / MAP_SIZE * IMAGE_SIZE;
        const yPos = (-(x.position.z - zMax)) / MAP_SIZE * IMAGE_SIZE;
        return `<circle r="1" cx="${xPos}" cy="${yPos}" fill="black">${animation(x.region_id)}</circle>`
    });

const content = `<svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 512 512"
>
    ${points.join('\n')}
</svg>`;

fs.writeFile('map.svg', content, err => {
    if (err) {
        console.error(err);
    } else {
        console.log('written')
    }
});

// https://slides.com/sarasoueidan/styling-animating-svgs-with-css#/10
// https://css-tricks.com/guide-svg-animations-smil/
