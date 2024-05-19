import { convertFileSrc } from "@tauri-apps/api/core";
import { LuHeart, LuImageOff, LuLock, LuUnlock } from "react-icons/lu";
import { Label } from "./ui/label";
import { Input } from "./ui/input";
import { Toggle } from "./ui/toggle";
import {
  Ref,
  forwardRef,
  useEffect,
  useImperativeHandle,
  useMemo,
  useRef,
  useState,
} from "react";
import { Tag } from "./tag-viewer";
import TagEditor from "./tag-editor";
import clsx from "clsx";
import { info } from "@tauri-apps/plugin-log";
import Combobox from "./ui/combobox";
import { concat, map } from "lodash/fp";
import { searchPackage } from "@/lib/cmd/library";

interface PackageComboboxItem{
  v: string,
  label: string
}

export interface StickyEditorRef {
  get lockName(): boolean;
  set lockName(value: boolean);
  get lockPackage(): boolean;
  set lockPackage(value: boolean);
  get lockedTags(): Tag[];
  set lockedTags(value: Tag[]);
  nameRef: Ref<HTMLInputElement | null>;
  pkgRef: Ref<HTMLInputElement | null>;
  reset: () => void;
}

interface StickyEditorProps {
  className?: string;
  path?: string;
  lockable?: boolean;

  fav?: boolean;
  onFavChanged?: (fav: boolean) => void;

  name?: string;
  onNameChanged?: (name: string) => void;

  pkg?: string;
  onPkgChanged?: (pkg: string) => void;

  tags?: Tag[];
  onTagsChanged?: (tags: Tag[]) => void;
}
const StickyEditor = forwardRef<StickyEditorRef, StickyEditorProps>(
  (
    {
      path,
      className,
      lockable,

      fav,
      onFavChanged,
      name,
      onNameChanged,
      pkg,
      onPkgChanged,
      tags,
      onTagsChanged,
    }: StickyEditorProps,
    ref
  ) => {
    const [lockName, setLockName] = useState(false);
    const [lockPackage, setLockPackage] = useState(false);
    const [lockedTags, setLockedTags] = useState<Tag[]>([]);
    const [pkgComboboxValue, setPkgComboboxValue] = useState<PackageComboboxItem>({
      v: pkg ?? "Inbox",
      label: pkg ?? "Inbox",
    });

    // make state sync with outside
    useEffect(() => {
      // not sync or not valid
      if (pkg != pkgComboboxValue.v || pkg.length == 0) {
        if (pkg && pkg.length != 0) {
          // outside state is valid
          // change state here
          setPkgComboboxValue({
            v: pkg,
            label: pkg,
          });
        } else {
          // outside state is not valid
          // fallback to default
          onPkgChanged && onPkgChanged("Inbox");
          setPkgComboboxValue({
            v: "Inbox",
            label: "Inbox",
          });
        }
      }
    }, [pkg]);

    const nameRef = useRef<HTMLInputElement>(null);
    const pkgRef = useRef<HTMLInputElement>(null);

    function reset() {
      info("reset sticky editor form");
      if (!lockName && nameRef.current) {
        nameRef.current.value = "";
        onNameChanged && onNameChanged("");
      }
      if (!lockPackage && pkgRef.current) {
        pkgRef.current.value = "";
        onPkgChanged && onPkgChanged("");
      }
      onTagsChanged && onTagsChanged(lockedTags);
    }

    useImperativeHandle(ref, () => ({
      get lockName() {
        return lockName;
      },
      set lockName(value: boolean) {
        setLockName(value);
      },
      get lockPackage() {
        return lockPackage;
      },
      set lockPackage(value: boolean) {
        setLockPackage(value);
      },
      get lockedTags() {
        return lockedTags;
      },
      set lockedTags(value: Tag[]) {
        setLockedTags(value);
      },
      nameRef,
      pkgRef,
      reset,
    }));

    const imagePreview = useMemo(() => {
      if (path) {
        const url = convertFileSrc(path);
        return <img src={url} />;
      } else {
        return <LuImageOff className="m-5" />;
      }
    }, [path]);

    return (
      <div className={className}>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
          <div className="bg-secondary rounded-lg border flex-1 flex items-center justify-center">
            {imagePreview}
          </div>
          <div className="flex-1 flex flex-col-reverse md:flex-col">
            <div className="grid items-center gap-2">
              <div className="flex flex-col space-y-1.5">
                <Label htmlFor="name">Name</Label>
                <div className="space-x-2 flex">
                  <Input
                    id="name"
                    ref={nameRef}
                    value={name}
                    onChange={(e) =>
                      onNameChanged && onNameChanged(e.target.value)
                    }
                  />
                  {!lockable ? null : (
                    <Toggle pressed={lockName} onPressedChange={setLockName}>
                      {lockName ? <LuLock /> : <LuUnlock />}
                    </Toggle>
                  )}
                </div>
              </div>

              <div className="flex flex-col space-y-1.5">
                <Label htmlFor="package">Package</Label>
                <div className="space-x-2 flex">
                  <Combobox
                    suggest={async (input) => {
                      let res = [{v: "Inbox", label: "Inbox"}]
                      if(input.trim().length > 0){
                        res.push({ v: input, label: `Create "${input}"` })
                      }
                      const localPkg = map((name)=>({v: name, label: name}),await searchPackage(input))
                      return concat(res, localPkg)
                    }}
                    comboboxRender={({ value }) => value?.label}
                    listItemRender={({ value }) => <span>{value?.label}</span>}
                    keyOf={(v) => v?.v ?? "__null"}
                    value={pkgComboboxValue}
                    onValueChanged={(newValue) => {
                      setPkgComboboxValue(newValue!);
                      onPkgChanged && onPkgChanged(newValue!.v);
                    }}
                  />

                  {!lockable ? null : (
                    <Toggle
                      pressed={lockPackage}
                      onPressedChange={setLockPackage}
                    >
                      {lockPackage ? <LuLock /> : <LuUnlock />}
                    </Toggle>
                  )}
                </div>
              </div>
            </div>
            <div>
              <Toggle pressed={fav} onPressedChange={onFavChanged}>
                <LuHeart className={clsx(fav ? "text-red-600" : null)} />
              </Toggle>
            </div>
          </div>
        </div>
        <div className="flex flex-col space-y-1.5 pt-2">
          <Label>Tag</Label>
          <TagEditor
            lockedTag={lockedTags}
            onLockedTagChanged={setLockedTags}
            lockable={lockable}
            tags={tags ?? []}
            onTagsChanged={onTagsChanged}
          />
        </div>
      </div>
    );
  }
);
StickyEditor.displayName = "StickyEditor";

export default StickyEditor;