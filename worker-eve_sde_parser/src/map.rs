use std::fs::File;
use std::io::Write;

use starfoundry_lib_types::RegionId;

use crate::parser::regions::Region;
use crate::parser::systems::System;
use crate::parser::stargate::Stargate;

const BASE_SVG: &str = r#"<svg
    xmlns="http://www.w3.org/2000/svg"
    style="background-color: #424242"
    viewBox="0 0 512 512"
    width="768px"
    height="768px"
    id="svg"
>
    <g id="g">
        <g id="regions" visibility="visible">
            {REPLACE_ME_FOR_REGION}
        </g>
        <g id="constellations" visibility="hidden">
            {REPLACE_ME_FOR_CONSTELLATION}
        </g>
        <g id="gates" visibility="hidden">
            {REPLACE_ME_FOR_GATES}
        </g>
    </g>
</svg>"#;

pub fn full_map(
    systems:    Vec<System>,
    stargates:  Vec<Stargate>,
    regions:    Vec<Region>,
) {
    let mut x_min = 0f64;
    let mut x_max = 0f64;
    let mut y_min = 0f64;
    let mut y_max = 0f64;

    for system in systems.iter() {
        let position = if let Some(x) = &system.position_2d {
            x
        } else {
            continue;
        };

        x_min = position.x.min(x_min);
        x_max = position.x.max(x_max);

        y_min = position.y.min(y_min);
        y_max = position.y.max(x_max);
    }

    let image_size = 512f64;
    let map_size = (x_max - x_min).min(y_max - y_min);
    dbg!(map_size);

    let ignored_regions = vec![
        RegionId(10000004),
        RegionId(10000017),
        RegionId(10000018),
        RegionId(10000019),
    ];
    let ignored_systems = systems
        .iter()
        .filter(|x| ignored_regions.contains(&x.region_id))
        .collect::<Vec<_>>();

    let points_regions = regions
        .iter()
        .filter(|x| !ignored_regions.contains(&x.region_id))
        .map(|x| {
            let x_pos = (x.position.x - x_min) / map_size * image_size;
            let y_pos = (-(x.position.z - y_max)) / map_size * image_size;

            format!(
                r#"<circle r="10" cx="{}" cy="{}" id="r{}" fill="red" />"#,
                x_pos,
                y_pos,
                x.region_id,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let points_constellation = systems
        .iter()
        .filter(|x| !ignored_regions.contains(&x.region_id))
        .filter(|x| x.position_2d.is_some())
        .map(|x| {
            // unwrap is safe as the filter above would have skipped this if
            // it's none
            //let position = x.position_2d.as_ref().unwrap();
            let position = x.position.clone();

            let x_pos = (position.x - x_min) / map_size * image_size;
            //let y_pos = (-(position.y - y_max)) / map_size * image_size;
            let y_pos = (-(position.z - y_max)) / map_size * image_size;

            format!(
                r#"<circle r="1" cx="{}" cy="{}" id="s{}" fill="blue" />"#,
                x_pos,
                y_pos,
                x.system_id,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let points_gates = stargates
        .iter()
        .filter(|x| ignored_systems.iter().find(|y| y.system_id == x.system_id).is_none())
        .map(|x| {
            let source = systems
                .iter()
                .find(|y| y.system_id == x.system_id)
                .unwrap()
                .position
                .clone();
                //.position_2d
                //.as_ref()
                //.unwrap();
            let x_pos_source = (source.x - x_min) / map_size * image_size;
            //let y_pos_source = (-(source.y - y_max)) / map_size * image_size;
            let y_pos_source = (-(source.z - y_max)) / map_size * image_size;

            let destination = systems
                .iter()
                .find(|y| y.system_id == x.destination.system_id)
                .unwrap()
                .position
                .clone();
                //.position_2d
                //.as_ref()
                //.unwrap();
            let x_pos_destination = (destination.x - x_min) / map_size * image_size;
            //let y_pos_destination = (-(destination.y - y_max)) / map_size * image_size;
            let y_pos_destination = (-(destination.z - y_max)) / map_size * image_size;

            format!(
                r#"<line r="1" x1="{}" y1="{}" x2="{}" y2="{}" id="g{}" style="stroke:red; stroke-width:0.1" />"#,
                x_pos_source,
                y_pos_source,
                x_pos_destination,
                y_pos_destination,
                x.stargate_id,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let full_map = BASE_SVG
        .replace("{REPLACE_ME_FOR_REGION}", &points_regions)
        .replace("{REPLACE_ME_FOR_CONSTELLATION}", &points_constellation)
        .replace("{REPLACE_ME_FOR_GATES}", &points_gates);
    let html = format!(r##"<html>
    <body>
        {full_map}
    </body>

    <script type="module">
        import * as d3 from "https://cdn.jsdelivr.net/npm/d3@7/+esm";

        const g = d3.select('#g');
        const svg = d3.select('#svg');

        const zoom = d3.zoom()
            .scaleExtent([1, 540])
            .on("zoom", zoomed);

        svg.call(zoom);

        function zoomed(event) {{
            console.log(event.transform.k)
            if (event.transform.k > 3) {{
                d3.select(this)
                    .select("#constellations")
                    .attr("visibility", "visible");
                d3.select(this)
                    .select("#gates")
                    .attr("visibility", "visible");
                d3.select(this)
                    .select("#regions")
                    .attr("visibility", "hidden");
            }} else {{
                d3.select(this)
                    .select("#constellations")
                    .attr("visibility", "hidden");
                d3.select(this)
                    .select("#gates")
                    .attr("visibility", "hidden");
                d3.select(this)
                    .select("#regions")
                    .attr("visibility", "visible");
            }}

            g.attr('transform', `translate(${{event.transform.x}}, ${{event.transform.y}}) scale(${{event.transform.k}})`);
        }};
    </script>
</html>"##);

    let mut file = File::create("index.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
}
