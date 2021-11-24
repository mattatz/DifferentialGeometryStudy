import React from "react";
import { useEffect, useState } from "react";
import { BufferGeometry, Color, DoubleSide, Float32BufferAttribute, Vector3 } from "three";
import { Lut } from "three/examples/jsm/math/Lut";
import { SurfaceTessellation } from "../../../wasm/pkg";
import { memory } from "../../../wasm/pkg/index_bg.wasm";

export const Curvature = {
  Gaussian: 'gaussian',
  Mean: 'mean',
} as const;
export type CurvatureType = typeof Curvature[keyof typeof Curvature];

const Surface = (props: {
  tessellation: SurfaceTessellation;
  surface: boolean;
  curvature?: CurvatureType;
  arrow: boolean;
  isoCurve: boolean;
  arrowLength: number;
}): JSX.Element => {
  const { tessellation, surface, curvature, arrow, arrowLength, isoCurve } = props;
  //const [pointsGeometry, setPointsGeometry] = useState<BufferGeometry>(new BufferGeometry());
  const [geometry, setGeometry] = useState<BufferGeometry>(new BufferGeometry());
  const [isNormal, setIsNormal] = useState<boolean>(false);
  const [arrowOrigin, setArrowOrigin] = useState<Vector3[]>([]);
  const [arrowDirection, setArrowDirection] = useState<Vector3[]>([]);
  const [isoGeometry, setIsoGeometry] = useState<BufferGeometry>(new BufferGeometry());

  const headLength = arrowLength * 0.2;
  const headWidth = arrowLength * 0.125;

  useEffect(() => {
    const stride = tessellation.stride();
    const pcount = tessellation.points_count();
    const icount = tessellation.indices_count();

    const pArray = new Float64Array(memory.buffer, tessellation.points(), pcount * stride);
    const nArray = new Float64Array(memory.buffer, tessellation.normals(), pcount * stride);
    const cArray = new Float64Array(
      memory.buffer,
      curvature === Curvature.Gaussian ? tessellation.gauss_curvature() : tessellation.mean_curvature(),
      pcount
    );

    const lut = new Lut('cooltowarm');
    const colors: number[] = [];

    let min = Number.MAX_VALUE;
    let max = Number.MIN_VALUE;
    for (let i = 0; i < pcount; i++) {
      const value = cArray[i];
      min =　Math.min(min, value);
      max = Math.max(max, value);
    }

    const interval = max - min;
    const threshold = 1e-6;
    if (interval <= threshold) {
      const c = lut.getColor(0.5);
      for (let i = 0; i < pcount; i++) {
        colors.push(c.r, c.g, c.b);
      }
    } else {
      for (let i = 0; i < pcount; i++) {
        const value = cArray[i];
        const c = lut.getColor((value - min) / interval);
        colors.push(c.r, c.g, c.b);
      }
    }

    const position = new Float32BufferAttribute(new Float32Array(pArray), 3);
    const normal = new Float32BufferAttribute(new Float32Array(nArray), 3);
    const iArray = new Int32Array(memory.buffer, tessellation.indices(), icount);

    const g = new BufferGeometry();
    g.setAttribute('position', position);
    // g.setAttribute('normal', normal);
    g.setAttribute('color', new Float32BufferAttribute(colors, 3));
    g.setIndex(Array.from(iArray));
    g.computeVertexNormals();
    setGeometry(g);

    const pts = [];
    const nms = [];
    for (let i = 0; i < pArray.length; i += 3) {
      pts.push(new Vector3(pArray[i], pArray[i + 1], pArray[i + 2]));
      nms.push(new Vector3(nArray[i], nArray[i + 1], nArray[i + 2]));
    }
    setArrowOrigin(pts);
    setArrowDirection(nms);

    const rows = tessellation.rows();
    const columns = tessellation.columns();

    const iPosition: number[] = [];
    const iColor: number[] = [];
    for (let u = 0; u < rows; u++) {
      const iu = u * columns;
      const fu = u / (rows - 1);
      for (let v = 0; v < columns - 1; v++) {
        const p0 = pts[iu + v];
        const p1 = pts[iu + v + 1];
        const fv0 = v / (columns - 1);
        const fv1 = (v + 1) / (columns - 1);
        iPosition.push(p0.x, p0.y, p0.z, p1.x, p1.y, p1.z);
        iColor.push(fu, fv0, 0, fu, fv1, 0);
      }
    }
    for (let v = 0; v < columns; v++) {
      const fv = v / (columns - 1);
      for (let u = 0; u < rows - 1; u++) {
        const p0 = pts[u * columns + v];
        const p1 = pts[(u + 1) * columns + v];
        const fu0 = u / (rows - 1);
        const fu1 = (u + 1) / (rows - 1);
        iPosition.push(p0.x, p0.y, p0.z, p1.x, p1.y, p1.z);
        iColor.push(fu0, fv, 0, fu1, fv, 0);
      }
    }
    const isog = new BufferGeometry();
    isog.setAttribute('position', new Float32BufferAttribute(iPosition, 3));
    isog.setAttribute('color', new Float32BufferAttribute(iColor, 3));
    setIsoGeometry(isog);
  }, [tessellation, curvature]);

  return (
    <group>
      {
        isoCurve ? 
          <lineSegments>
            <primitive object={isoGeometry} attach="geometry" />
            <lineBasicMaterial attach="material" color={'#ffffff'} transparent={true} opacity={0.65} vertexColors={true}></lineBasicMaterial>
          </lineSegments>
        : <></>
      }
      {
        arrow ? 
          arrowOrigin.map((p, i) => {
            return <arrowHelper
              key={ `arrow-${i}` }
              args={ [arrowDirection[i], arrowOrigin[i], arrowLength, (function() {
                const c = arrowDirection[i].clone().addScalar(1.0).multiplyScalar(0.5);
                return new Color(c.x, c.y, c.z);
              })(), headLength, headWidth ] }
            ></arrowHelper>
          }) : <></>
      }
      {
        surface ?
          <mesh>
            <primitive object={geometry} attach="geometry" />
            {
              isNormal ?
                <meshNormalMaterial attach="material" side={ DoubleSide }></meshNormalMaterial> :
                // <meshBasicMaterial attach="material" vertexColors={true} side={ DoubleSide }></meshBasicMaterial>
                <meshStandardMaterial attach="material" roughness={1.0} metalness={0.0} color={ new Color(0xffffff) } vertexColors={true} side={ DoubleSide }></meshStandardMaterial>
            }
          </mesh>
        : <></>
      }
    </group>
  );
}

export {
  Surface
};
