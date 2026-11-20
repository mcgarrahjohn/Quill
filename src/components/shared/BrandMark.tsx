import React from "react";
import QuillFeather from "../icons/QuillFeather";

interface BrandMarkProps {
  /** Height in px. */
  size: number;
  /** Accepted for the onboarding screens that still pass it; the mark stays still. */
  animation?: "wave" | "slap" | null;
}

// The feather mark's frame is 18 by 20.
const BOX_ASPECT = 18 / 20;

/** The Quill feather mark, in the brand amber. */
export const BrandMark: React.FC<BrandMarkProps> = ({ size }) => (
  <span
    style={{ width: size * BOX_ASPECT, height: size }}
    className="relative block select-none"
  >
    <QuillFeather
      width={size * BOX_ASPECT}
      height={size}
      className="stroke-background-ui"
    />
  </span>
);
