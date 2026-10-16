import { EN, type GuiStrings } from "./i18n";
import type { KeyboardEvent } from "react";
import { trapTab } from "./ConfirmDialog";
import "./Dialog.css";

/** The "Create playlist" dialog: a name field, Create and Cancel. The caller holds the name. */
export function PlaylistNameDialog({
  s = EN,
  name,
  onName,
  onCreate,
  onCancel,
}: {
  s?: GuiStrings;
  name: string;
  onName: (name: string) => void;
  onCreate: (name: string) => void;
  onCancel: () => void;
}) {
  const blank = name.trim() === "";
  const create = () => {
    if (!blank) onCreate(name.trim());
  };
  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    event.stopPropagation();
    if (event.nativeEvent.isComposing) return;
    trapTab(event);
    if (event.key === "Escape") onCancel();
    else if (
      event.key === "Enter" &&
      (event.target as HTMLElement).tagName === "INPUT"
    )
      create();
    else return;
    event.preventDefault();
  };
  return (
    <div className="dialog-scrim" onClick={onCancel}>
      <div
        className="dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby="playlist-name-title"
        onClick={(event) => event.stopPropagation()}
        onKeyDown={onKeyDown}
      >
        <h2 id="playlist-name-title">{s.create_playlist}</h2>
        <label className="dialog-field">
          <span>{s.name}</span>
          <input
            type="text"
            autoFocus
            value={name}
            placeholder={s.my_playlist}
            onChange={(event) => onName(event.target.value)}
          />
        </label>
        <div className="dialog-buttons">
          <button type="button" className="dialog-cancel" onClick={onCancel}>
            {s.cancel}
          </button>
          <button
            type="button"
            className="dialog-confirm"
            disabled={blank}
            onClick={create}
          >
            {s.create}
          </button>
        </div>
      </div>
    </div>
  );
}
