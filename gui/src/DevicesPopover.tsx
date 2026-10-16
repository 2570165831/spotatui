import { deviceKind, EN, type GuiStrings } from "./i18n";
import { useEffect, useLayoutEffect, useRef } from "react";
import type { Action } from "./bindings/Action";
import type { DeviceInfo } from "./bindings/DeviceInfo";
import { Glyph } from "./Icon";

/** The device picker above the devices button. Clicks inside its parent, the button's box, are not outside clicks. */
export function DevicesPopover({
  s = EN,
  devices,
  current,
  send,
  onClose,
}: {
  s?: GuiStrings;
  devices: DeviceInfo[];
  /** The name of the device that plays now. */
  current: string | null;
  send: (action: Action) => void;
  onClose: () => void;
}) {
  const root = useRef<HTMLDivElement>(null);
  useEffect(() => {
    send("RefreshDevices");
  }, [send]);
  useLayoutEffect(() => {
    root.current?.querySelector<HTMLElement>("button:not(:disabled)")?.focus();
  }, []);
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.preventDefault();
      onClose();
    };
    const onPointerDown = (event: PointerEvent) => {
      const box = root.current?.parentElement ?? root.current;
      if (event.target instanceof Node && box?.contains(event.target)) return;
      onClose();
    };
    document.addEventListener("keydown", onKeyDown);
    document.addEventListener("pointerdown", onPointerDown);
    return () => {
      document.removeEventListener("keydown", onKeyDown);
      document.removeEventListener("pointerdown", onPointerDown);
    };
  }, [onClose]);

  const active = devices.find((device) => device.is_active)?.name ?? current;
  const others = devices.filter(
    (device) => !device.is_active && device.name !== active,
  );
  return (
    <div
      ref={root}
      className="devices-popover"
      role="dialog"
      aria-label={s.devices}
    >
      <h2>{s.connect_device}</h2>
      {active && (
        <div className="device-current">
          <Glyph name="devices" size={28} />
          <span>
            <strong>{s.current_device}</strong>
            <span>{active}</span>
          </span>
        </div>
      )}
      <h3>{s.select_other_device}</h3>
      {others.length === 0 ? (
        <p className="device-empty">{s.no_other_devices}</p>
      ) : (
        <ul>
          {others.map((device) => (
            <li key={device.id ?? device.name}>
              <button
                type="button"
                disabled={device.id === null}
                onClick={() => {
                  if (device.id === null) return;
                  send({
                    TransferPlayback: { device_id: device.id, persist: false },
                  });
                  onClose();
                }}
              >
                <Glyph name="devices" size={20} />
                <span>
                  <strong>{device.name}</strong>
                  <span>{deviceKind(device.kind, s)}</span>
                </span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
