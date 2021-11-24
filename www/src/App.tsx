import React, { useEffect, useMemo, useRef, useState } from "react";
import dat, { GUI } from 'dat.gui'
import { Canvas, useFrame } from '@react-three/fiber'
import { BufferAttribute, BufferGeometry, Color, DirectionalLight, DoubleSide, Float32BufferAttribute, GeometryUtils, Int16BufferAttribute, Int32BufferAttribute, MeshNormalMaterial, MeshStandardMaterial, PlaneGeometry, Sphere, Vector3 } from "three";
import { Controls } from "./Control";

import * as wasm from "../../wasm/pkg";
import { memory } from "../../wasm/pkg/index_bg.wasm";
import styled from "styled-components";
import { CurveTessellation, SurfaceTessellation } from "../../wasm/pkg";
import { FrenetFrames } from "./preview/FrenetFrames";
import { Curvature, CurvatureType, Surface } from "./preview/Surface";

const App = () => {
  const light = useRef<DirectionalLight>(null!);
  const [edges, setEdges] = useState<CurveTessellation[]>([]);
  const [surfaces, setSurfaces] = useState<SurfaceTessellation[]>([]);
  const size = 20;
  const [grid, setGrid] = useState<boolean>(false);

  const [surfaceType, setSurfaceType] = useState<wasm.SurfaceType>(wasm.SurfaceType.Sphere);
  const [useSurface, setUseSurface] = useState<boolean>(true);
  const [curvature, setCurvature] = useState<CurvatureType>(Curvature.Gaussian);
  const [useArrow, setUseArrow] = useState<boolean>(false);
  const [useIsoCurve, setUseIsoCurve] = useState<boolean>(true);

  const app = useMemo(() => {
    const app = wasm.App.new();

    /*
    const n = app.curves_count();
    const e: wasm.CurveTessellation[] = Array.from(Array(n).keys()).map(i => {
      return app.tessellate_curve(i, delta);
    }).filter(e => e !== undefined) as wasm.CurveTessellation[];
    setEdges(e);
    */

    return app;
  }, []);

  useEffect(() => {
    const delta = 1e-2 * 2.0;
    const s = app.create_surface(Number(surfaceType), delta);
    if (s !== undefined) {
      setSurfaces([s]);
    }
  }, [surfaceType]);

  const gui = useMemo(() => {
    const gui = new dat.GUI({ name: 'Differential Geometry Study', width: 300 });
    const types = Object.keys(wasm.SurfaceType).filter(k => !isNaN(Number(k))).map(k => {
      const i = Number(k);
      return { index: k, value: wasm.SurfaceType[i] };
    }).reduce((pre: { [index: string]: string }, cur) => {
      const { index, value } = cur;
      pre[value] = index;
      return pre;
    }, {});
    gui.add({ surfaceType }, 'surfaceType', types).name('type').onChange(setSurfaceType);
    gui.add({ useSurface }, 'useSurface').name('surface').onChange(setUseSurface);
    gui.add({ curvature }, 'curvature', Curvature).onChange(setCurvature);
    gui.add({ useArrow }, 'useArrow').name('arrow').onChange(setUseArrow);
    gui.add({ useIsoCurve }, 'useIsoCurve').name('iso curve').onChange(setUseIsoCurve);
    return gui;
  }, [])

  return (
    <>
      <div ref={ref => ref?.appendChild(gui.domElement)} style={{position: "absolute", zIndex: 1e5}} />
      <Canvas style={ { height: "100vh" } } orthographic
        shadows={true}
        camera={{ position: [100, 100, 100], zoom: 100, near: 0.1, far:1000 }}
      >
        <Controls onUpdate={ (controls) => {
          light.current?.position.copy(controls.object.position);
        } } />
        <ambientLight intensity={0.25} />
        <directionalLight ref={light} intensity={0.7} position={[10, 5, 10]} castShadow={ false } />
        <group>
          <group
            rotation={[-Math.PI * 0.5, 0, 0]}
          >
            {
              edges.map((edge, i) => {
                return <FrenetFrames key={ `edge-${i}` } edge={edge} length={0.05}></FrenetFrames>
              })
            }
            {
              surfaces.map((tess, i) => {
                return <Surface key={ `surface-${i}` } tessellation={tess} surface={ useSurface } curvature={ curvature } arrow={ useArrow } isoCurve={ useIsoCurve } arrowLength={0.2}></Surface>
              })
            }
          </group>
          {
            grid ? <group>
              <gridHelper args={[size, size, `gray`, `gray`]} />
              <gridHelper position={[-size * 0.5, size * 0.5, 0]} rotation={[0, 0, Math.PI * 0.5]} args={[size, size, `gray`, `gray`]} />
              <gridHelper position={[0, size * 0.5, -size * 0.5]} rotation={[Math.PI * 0.5, 0, 0]} args={[size, size, `gray`, `gray`]} />
            </group> : <></>
          }
        </group>
      </Canvas>
    </>
  );
};

export default App;