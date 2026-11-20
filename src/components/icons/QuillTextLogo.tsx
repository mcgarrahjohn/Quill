/**
 * The Quill wordmark in the app's display face. `withMark` adds the favicon
 * feather before the word with the same proportions as the Quill website's
 * logo: 1.6x the height of the letters, centered on them. Without it, the
 * frame spans the letters (baseline to top), so a feather placed beside this
 * wordmark and centered on it matches the website lockup.
 */
const BRAND_NAME = "Quill";

const QuillTextLogo = ({
  width,
  height,
  className,
  withMark = false,
}: {
  width?: number;
  height?: number;
  className?: string;
  withMark?: boolean;
}) => {
  const textX = withMark ? 151 : 0;
  return (
    <svg
      width={width}
      height={height}
      className={className}
      style={{ overflow: "visible" }}
      viewBox={withMark ? "0 -6 366 144" : "0 22 212 86"}
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      role="img"
      aria-label={BRAND_NAME}
    >
      {withMark && (
        <g
          stroke="var(--color-background-ui)"
          strokeWidth="1.4"
          strokeLinecap="round"
          strokeLinejoin="round"
          transform="translate(-28.8 -24.5) scale(9.05)"
        >
          <path d="M3.625 17.38L3.8 14.6C4.209 8.11 8.975 2.62 15.475 2.62C15.833 2.62 16.156 2.84 16.3 3.16C16.443 3.49 16.378 3.87 16.135 4.14L12.28 8.3C13.38 8.4 14.4 8.65 14.4 9.65C14.4 12.25 8.8 14.6 3.8 14.6" />
        </g>
      )}
      <text
        x={textX}
        y="108"
        fill="var(--color-text)"
        fontSize="112"
        fontWeight="650"
        letterSpacing="-3"
        style={{ fontFamily: "var(--font-display)" }}
      >
        {BRAND_NAME}
      </text>
    </svg>
  );
};

export default QuillTextLogo;
