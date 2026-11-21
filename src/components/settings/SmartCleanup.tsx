import React, { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { listen } from "@tauri-apps/api/event";
import { commands, type CleanupStatus } from "@/bindings";
import { ToggleSwitch } from "../ui/ToggleSwitch";
import { Button } from "../ui/Button";
import { useSettings } from "../../hooks/useSettings";

interface SmartCleanupProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

const GB = 1024 ** 3;

/**
 * "Fix filler words and self-corrections": an optional on-device cleanup.
 * Off by default. Turning it on downloads the model once (about 1 GB); turning
 * it off frees the memory it uses.
 */
export const SmartCleanup: React.FC<SmartCleanupProps> = React.memo(
  ({ descriptionMode = "inline", grouped = true }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();
    const enabled = getSetting("smart_cleanup_enabled") ?? false;
    const [status, setStatus] = useState<CleanupStatus | null>(null);

    const refresh = useCallback(async () => {
      try {
        setStatus(await commands.getCleanupStatus());
      } catch (e) {
        console.warn("Could not read the cleanup status:", e);
      }
    }, []);

    useEffect(() => {
      refresh();
      const unlisten = listen<CleanupStatus>(
        "cleanup-status-changed",
        (event) => setStatus(event.payload),
      );
      return () => {
        unlisten.then((fn) => fn());
      };
    }, [refresh]);

    // The setting changed from here (or elsewhere): re-read what the backend is doing.
    useEffect(() => {
      refresh();
    }, [enabled, refresh]);

    const percent =
      status && status.total_bytes > 0
        ? Math.min(
            100,
            Math.round((status.downloaded_bytes / status.total_bytes) * 100),
          )
        : 0;

    let detail: React.ReactNode = null;
    if (status && enabled) {
      if (status.downloading) {
        detail = (
          <div className="flex items-center gap-3">
            <div className="h-1.5 flex-1 rounded-full bg-line overflow-hidden">
              <div
                className="h-full rounded-full bg-background-ui transition-[width]"
                style={{ width: `${percent}%` }}
              />
            </div>
            <span className="tabular-nums text-ink-2">
              {t("settings.cleanup.downloading", { percent })}
            </span>
            <Button
              variant="secondary"
              size="sm"
              onClick={() => commands.cancelCleanupDownload()}
            >
              {t("settings.cleanup.cancel")}
            </Button>
          </div>
        );
      } else if (status.error) {
        detail = (
          <div className="flex items-center gap-3">
            <span className="flex-1 text-error">
              {t("settings.cleanup.failed", { error: status.error })}
            </span>
            <Button
              variant="secondary"
              size="sm"
              onClick={() => commands.startCleanupDownload()}
            >
              {t("settings.cleanup.retry")}
            </Button>
          </div>
        );
      } else if (!status.files_ready) {
        detail = (
          <div className="flex items-center gap-3">
            <span className="flex-1 text-ink-2">
              {t("settings.cleanup.needsDownload")}
            </span>
            <Button
              variant="secondary"
              size="sm"
              onClick={() => commands.startCleanupDownload()}
            >
              {t("settings.cleanup.download")}
            </Button>
          </div>
        );
      } else {
        detail = (
          <span className="text-ink-2">{t("settings.cleanup.ready")}</span>
        );
      }
    } else if (status && !enabled && status.files_ready) {
      // Off, but the file is still on disk: offer to give the space back.
      detail = (
        <div className="flex items-center gap-3">
          <span className="flex-1 text-ink-2">
            {t("settings.cleanup.onDisk", {
              size: (status.total_bytes / GB).toFixed(1),
            })}
          </span>
          <Button
            variant="secondary"
            size="sm"
            onClick={async () => {
              await commands.deleteCleanupModel();
              refresh();
            }}
          >
            {t("settings.cleanup.remove")}
          </Button>
        </div>
      );
    }

    return (
      <>
        <ToggleSwitch
          checked={enabled}
          onChange={(value) => updateSetting("smart_cleanup_enabled", value)}
          isUpdating={isUpdating("smart_cleanup_enabled")}
          label=""
          ariaLabel={t("settings.cleanup.label")}
          description={t("settings.cleanup.description")}
          descriptionMode={descriptionMode}
          grouped={grouped}
        />
        {detail && <div className="px-4 pb-3 pt-1 text-xs">{detail}</div>}
      </>
    );
  },
);
