const fs = require('node:fs');
const map_data = require('../map.json');

let xMin = 0;
let xMax = 0;
let yMin = 0;
let yMax = 0;

const timings = {};
map_data
    .forEach(x => {
        const start = Math.random() * 3;
        timings[x.region_id] = start;
    });

map_data
    .forEach(x => {
        xMin = Math.min(xMin, x.position2D.x);
        xMax = Math.max(xMax, x.position2D.x);
        yMin = Math.min(yMin, x.position2D.y);
        yMax = Math.max(yMax, x.position2D.y);
    });

const IMAGE_SIZE = 512;
const MAP_SIZE = Math.max((xMax - xMin), (yMax - yMin));

const animation = (region_id) => {
    return `<animate
        attributeType="xml"
        attributeName="fill-opacity"
        begin="${timings[region_id]}s"
        values="1;0.75;0.5;0.25;0.5;0.75"
        dur="6s"
        repeatCount="indefinite"
    />`
}

let abe_x = 0;
let abe_y = 0;
let ualx_x = 0;
let ualx_y = 0;

const points = map_data
    .map(x => {
        const xPos = (x.position2D.x - xMin) / MAP_SIZE * IMAGE_SIZE;
        const yPos = (-(x.position2D.y - yMax)) / MAP_SIZE * IMAGE_SIZE;
        let color;

        if (x.system_id === 30004807) {
            ualx_x = xPos;
            ualx_y = yPos;
        }
        if (x.system_id === 30004831) {
            abe_x = xPos;
            abe_y = yPos;
        }

        // colors taken from wikipedia
        // https://en.wikipedia.org/wiki/Stellar_classification
        const temperature = x.star.temperature;
        if (temperature >= 2300 && temperature <= 3900) {
            color = '#ffcc6f'
        } else if (temperature > 3900 && temperature <= 5300) {
            color = '#ffd2a1'
        } else if (temperature > 5300 && temperature <= 6000) {
            color = '#fff4ea'
        } else if (temperature > 6000 && temperature <= 7300) {
            color = '#f8f7ff'
        } else if (temperature > 7300 && temperature <= 10000) {
            color = '#cad7ff'
        } else if (temperature > 10000 && temperature <= 33000) {
            color = '#aabfff'
        } else {
            color = '#9bb0ff'
        }

        return `<circle r="1" cx="${xPos}" cy="${yPos}" fill="${color}" id="s${x.system_id}">${animation(x.region_id)}</circle>`
    });

const viewBoxDefault = 'viewBox="0 0 512 512"'

const content = `<svg
    xmlns="http://www.w3.org/2000/svg"
    style="background-color: #424242"
    viewBox="0 0 512 512"
    width="512px"
    height="512px"
    id="svg"
>
    <g>
        <g id="g">
            ${points.join('\n')}
        </g>
    </g>
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
