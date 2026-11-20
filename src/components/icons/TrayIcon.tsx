/**
 * The tray icon: the Quill feather, plus a dot while recording and three dots
 * while transcribing. Mirrors src-tauri/resources/tray_*.png; uses currentColor
 * like a macOS template image.
 */
type TrayIconState = "idle" | "recording" | "transcribing";

const FEATHER =
  "M3.625 17.38L3.8 14.6C4.209 8.11 8.975 2.62 15.475 2.62C15.833 2.62 16.156 2.84 16.3 3.16C16.443 3.49 16.378 3.87 16.135 4.14L12.28 8.3C13.38 8.4 14.4 8.65 14.4 9.65C14.4 12.25 8.8 14.6 3.8 14.6";

const TrayIcon = ({
  state,
  size,
}: {
  state: TrayIconState;
  size: number | string;
}) => {
  const idle = state === "idle";
  const id = `quill-tray-${state}`;
  // Idle: the feather fills the frame. Otherwise it steps aside for the badge.
  const transform = idle
    ? "translate(-3.2 -3.2) scale(3.52)"
    : "translate(-2.13 -0.13) scale(2.913)";
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      {!idle && (
        <defs>
          <mask
            id={id}
            maskUnits="userSpaceOnUse"
            x="0"
            y="0"
            width="64"
            height="64"
          >
            <rect width="64" height="64" fill="white" />
            {state === "recording" ? (
              <circle cx="48.5" cy="48.5" r="14.5" fill="black" />
            ) : (
              <rect
                x="32"
                y="41.5"
                width="34"
                height="16"
                rx="7"
                fill="black"
              />
            )}
          </mask>
        </defs>
      )}
      <g mask={idle ? undefined : `url(#${id})`}>
        <path
          d={FEATHER}
          transform={transform}
          stroke="currentColor"
          strokeWidth="1.65"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </g>
      {state === "recording" && (
        <circle cx="48.5" cy="48.5" r="11" fill="currentColor" />
      )}
      {state === "transcribing" && (
        <g fill="currentColor">
          <circle cx="39.5" cy="49.5" r="3.6" />
          <circle cx="49" cy="49.5" r="3.6" />
          <circle cx="58.5" cy="49.5" r="3.6" />
        </g>
      )}
    </svg>
  );
};

export default TrayIcon;
