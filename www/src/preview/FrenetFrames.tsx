import React from "react";
import { useEffect, useState } from "react";
import { BufferGeometry, Color, Float32BufferAttribute, Vector3 } from "three";
import { CurveTessellation } from "../../../wasm/pkg";
import { memory } from "../../../wasm/pkg/index_bg.wasm";
import { CurvatureRadiusCircle } from "./CurvatureRadiusCircle";

const FrenetFrames = (props: {
  edge: CurveTessellation;
  length: number;
}): JSX.Element => {
  const { edge, length } = props;
  const [pointsGeometry, setPointsGeometry] = useState<BufferGeometry>(new BufferGeometry());

  const [origins, setOrigins] = useState<Vector3[]>([]);
  const [tangents, setTangents] = useState<Vector3[]>([]);
  const [normals, setNormals] = useState<Vector3[]>([]);
  const [binormals, setBinormals] = useState<Vector3[]>([]);
  const [curvatures, setCurvatures] = useState<number[]>([]);

  useEffect(() => {
    const count = edge.count();
    const len = count;
    const stride = edge.stride();

    const point = new Float32Array(memory.buffer, edge.points(), len * stride);
    const tangent = new Float32Array(memory.buffer, edge.tangents(), len * stride);
    const normal = new Float32Array(memory.buffer, edge.normals(), len * stride);
    const binormal = new Float32Array(memory.buffer, edge.binormals(), len * stride);
    const curvature = new Float32Array(memory.buffer, edge.curvatures(), len);

    const g = new BufferGeometry();
    const position = new Float32BufferAttribute(point, 3);
    g.setAttribute('position', position);
    setPointsGeometry(g);

    const o: Vector3[] = [];
    const t: Vector3[] = [];
    const n: Vector3[] = [];
    const b: Vector3[] = [];
    const c: number[] = [];
    for (let i = 0; i < len; i += 1) {
      const ia = i * stride;
      const ib = ia + 1;
      const ic = ia + 2;
      o.push(new Vector3(point[ia], point[ib], point[ic]));
      t.push(new Vector3(tangent[ia], tangent[ib], tangent[ic]));
      n.push(new Vector3(normal[ia], normal[ib], normal[ic]));
      b.push(new Vector3(binormal[ia], binormal[ib], binormal[ic]));
      c.push(curvature[i]);
    }

    setOrigins(o);
    setTangents(t);
    setNormals(n);
    setBinormals(b);
    setCurvatures(c);
  }, [edge]);

  const headLength = length * 0.2;
  const headWidth = length * 0.125;

  return <group>
    <points>
      <primitive object={pointsGeometry} attach="geometry" />
      <pointsMaterial attach="material" color={'#ff0000'} size={5}></pointsMaterial>
    </points>
    {
      origins.map((p, i) => {
        return <arrowHelper
          key={ `t-${i}` }
          args={ [tangents[i], origins[i], length, new Color(0xff2200), headLength, headWidth ] }
        ></arrowHelper>
      })
    }
    {
      origins.map((p, i) => {
        return <arrowHelper
          key={ `n-${i}` }
          args={ [normals[i], origins[i], length, new Color(0x22ff00), headLength, headWidth ] }
        ></arrowHelper>
      })
    }
    {
      origins.map((p, i) => {
        return <arrowHelper
          key={ `b-${i}` }
          args={ [binormals[i], origins[i], length, new Color(0x3100ff), headLength, headWidth ] }
        ></arrowHelper>
      })
    }
    {
      origins.map((p, i) => {
        return <CurvatureRadiusCircle
          key={ `curvature-${i}` }
          origin={p}
          tangent={tangents[i]}
          normal={normals[i]}
          curvature={curvatures[i] * 10.0}
          resolution={32}
          color={ new Color(0x4492f3) }
        ></CurvatureRadiusCircle>
      })
    }
  </group>;
}

export {
  FrenetFrames
};
