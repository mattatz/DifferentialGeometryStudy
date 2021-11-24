import React from "react";
import { useEffect, useState } from "react";
import { BufferGeometry, Color, Vector3 } from "three";

const CurvatureRadiusCircle = (props: {
  origin: Vector3;
  tangent: Vector3;
  normal: Vector3;
  curvature: number;
  resolution: number;
  color: Color;
}): JSX.Element => {
  const { origin, normal, tangent, curvature, resolution, color } = props;
  const [lineGeometry, setLineGeometry] = useState<BufferGeometry>(new BufferGeometry());
  const tau = Math.PI * 2;

  useEffect(() => {
    const g = new BufferGeometry();
    const radius = 1 / curvature;
    const o = origin.clone().add(normal.clone().setLength(radius));
    const points: Vector3[] = [];
    for (let i = 0; i < resolution; i++) {
      const t = i / (resolution - 1) * tau;
      const dx = Math.cos(t) * radius;
      const dy = Math.sin(t) * radius;
      const p = o.clone().add(normal.clone().setLength(dx)).add(tangent.clone().setLength(dy));
      points.push(p);
    }
    g.setFromPoints(points);
    setLineGeometry(g);
  }, [
    origin, normal, tangent, curvature
  ]);

  return (
    <line>
      <primitive object={lineGeometry} attach="geometry" />
      <lineBasicMaterial attach="material" color={ color } linewidth={10} linecap={'round'} linejoin={'round'} />
    </line>
  );
}

export {
  CurvatureRadiusCircle
};
