/**
 * Quill's feather mark: the same outline as the Quill favicon, geometry
 * included (32-unit frame, `translate(4 4) scale(1.2)`, 1.35 stroke). The
 * viewBox only crops away the favicon's tile.
 */
const QuillFeather = ({
  width,
  height,
  className,
}: {
  width?: number | string;
  height?: number | string;
  className?: string;
}) => (
  <svg
    width={width || 54}
    height={height || 60}
    viewBox="7 6 18 20"
    className={
      /\bstroke-/.test(className ?? "")
        ? className
        : `stroke-text ${className ?? ""}`
    }
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    aria-hidden="true"
  >
    <g
      strokeWidth="1.35"
      strokeLinecap="round"
      strokeLinejoin="round"
      transform="translate(4 4) scale(1.2)"
    >
      <path d="M3.625 17.38L3.8 14.6C4.209 8.11 8.975 2.62 15.475 2.62C15.833 2.62 16.156 2.84 16.3 3.16C16.443 3.49 16.378 3.87 16.135 4.14L12.28 8.3C13.38 8.4 14.4 8.65 14.4 9.65C14.4 12.25 8.8 14.6 3.8 14.6" />
    </g>
  </svg>
);

export default QuillFeather;
