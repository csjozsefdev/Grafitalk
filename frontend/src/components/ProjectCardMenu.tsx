import { useEffect, useRef } from "react";

interface ProjectCardMenuProps {
  projectName: string;
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onRename: () => void;
  onRemove: () => void;
  disabled?: boolean;
  removing?: boolean;
}

export function ProjectCardMenu({
  projectName,
  isOpen,
  onOpenChange,
  onRename,
  onRemove,
  disabled = false,
  removing = false,
}: ProjectCardMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    const handlePointerDown = (event: MouseEvent) => {
      if (
        menuRef.current &&
        !menuRef.current.contains(event.target as Node)
      ) {
        onOpenChange(false);
      }
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        onOpenChange(false);
      }
    };

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [isOpen, onOpenChange]);

  return (
    <div className="gt-project-item__menu" ref={menuRef}>
      <button
        type="button"
        className="gt-project-menu__trigger"
        aria-label={`Actions for ${projectName}`}
        aria-haspopup="menu"
        aria-expanded={isOpen}
        disabled={disabled}
        onClick={(event) => {
          event.stopPropagation();
          onOpenChange(!isOpen);
        }}
      >
        <span aria-hidden="true">⋯</span>
      </button>
      {isOpen ? (
        <ul className="gt-project-menu" role="menu">
          <li role="none">
            <button
              type="button"
              role="menuitem"
              className="gt-project-menu__item"
              disabled={disabled}
              onClick={() => {
                onOpenChange(false);
                onRename();
              }}
            >
              Rename
            </button>
          </li>
          <li role="none">
            <button
              type="button"
              role="menuitem"
              className="gt-project-menu__item gt-project-menu__item--destructive"
              disabled={disabled || removing}
              onClick={() => {
                onOpenChange(false);
                onRemove();
              }}
            >
              {removing ? "…" : "Remove"}
            </button>
          </li>
        </ul>
      ) : null}
    </div>
  );
}
