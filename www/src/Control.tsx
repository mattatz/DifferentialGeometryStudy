import React, { useEffect, useRef } from 'react';
import { ReactThreeFiber, extend, useThree, useFrame } from '@react-three/fiber';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

extend({ OrbitControls });

declare global {
  namespace JSX {
    interface IntrinsicElements {
      orbitControls: ReactThreeFiber.Node<OrbitControls, typeof OrbitControls>
    }
  }
}

type ControlProps = {
  onUpdate?: (controls: OrbitControls) => void;
}

export const Controls: React.FC<ControlProps> = (props: ControlProps) => {
  const controlsRef = useRef<OrbitControls>();
  const { camera, gl } = useThree();
  const { onUpdate } = props;

  useEffect(() => {
    const current = controlsRef.current;
    if (current !== undefined) {
      current.addEventListener('change', () => {
        if (onUpdate !== undefined) {
          onUpdate(current);
        }
      });
    }
  }, [controlsRef]);

  useFrame(() => {
    controlsRef.current?.update();
  });

  return (
    <orbitControls
      ref={controlsRef}
      args={[camera, gl.domElement]}
      enableZoom={true}
      zoomSpeed={1.0}
      enableRotate={true}
      rotateSpeed={1.0}
      enablePan={true}
      panSpeed={2.0}
      minZoom={1}
      maxZoom={1e3}
      minPolarAngle={0}
      maxPolarAngle={Math.PI}
    />
  );
}