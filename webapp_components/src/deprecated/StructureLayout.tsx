import type { Structure } from "@internal/services/structure/list";
import { useEffect, useRef, useState } from "react";

export function StructureLayout({
    structures,
}: Props) {
    const BORDER = 5;
    const STRUCTURE_BOX_SIZE = 50;

    const svgRef = useRef<SVGSVGElement>(null);

    const [imageSizeX, setImageSizeX] = useState(500);
    const [imageSizeY, setImageSizeY] = useState(60);

    useEffect(() => {
        setTimeout(() => {
            updateSize();
        }, 0);

        const updateSize = () => {
            if (svgRef && svgRef.current) {
                let { width, height } = svgRef.current.getBoundingClientRect();
                setImageSizeX(width - STRUCTURE_BOX_SIZE - (BORDER * 2));
                setImageSizeY(height);
            }
        };

        window.addEventListener("resize", updateSize);
        return () => window.removeEventListener("resize", updateSize);
    }, [svgRef]);

    const positionX = (structure: Structure) => {
        if (structure.position.x < 0) {
            return structure.position.x * -1;
        } else {
            return structure.position.x;
        }
    }
    const positionZ = (structure: Structure) => {
        if (structure.position.z < 0) {
            return structure.position.z * -1;
        } else {
            return structure.position.z;
        }
    }

    let minX = positionX(structures[0]);
    let maxX = positionX(structures[0]);
    let minZ = positionZ(structures[0]);
    let maxZ = positionZ(structures[0]);

    for (const structure of structures) {
        minX = Math.min(minX, positionX(structure));
        maxX = Math.max(maxX, positionX(structure));
        minZ = Math.max(minZ, positionZ(structure));
        maxZ = Math.min(maxZ, positionZ(structure));
    }

    let mapSize = Math.max((maxX - minX), (maxZ - minZ)) - BORDER;

    const sorted = structures
        .sort((a, b) => b.position.x - a.position.x)
        .map(x => {
            const mapX = (positionX(x) - minX) / mapSize * imageSizeX;

            return <>
                <image
                    style={{
                        width: `${STRUCTURE_BOX_SIZE}px`,
                        height: `${STRUCTURE_BOX_SIZE}px`,
                    }}
                    x={`${mapX + BORDER}px`}
                    y={`${BORDER}px`}
                    href={ `https://images.evetech.net/types/${x.item.type_id}/icon` }
                />
            </>
        });

    return <>
        <svg
            style={{
                width: `100%`,
                height: `${imageSizeY}px`,
                backgroundColor: '#1f1f1f',
            }}
            ref={svgRef}
        >
            { sorted }
        </svg>
    </>
}

export type Props = {
    structures: Structure[],
};
