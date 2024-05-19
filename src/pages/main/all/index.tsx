import StickyList from "@/components/sticky-list";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { StickyThumb, searchSticky } from "@/lib/cmd/library";
import { popModalAtom, pushDialogAtom } from "@/store/modal";
import { useSetAtom } from "jotai";
import { lazy, useEffect, useRef, useState } from "react";
import {
  LuImagePlus,
  LuMenu,
  LuSearch,
  LuSearchSlash,
  LuTextCursorInput,
} from "react-icons/lu";

export default function AllPage() {
  const pushDialog = useSetAtom(pushDialogAtom);
  const popModal = useSetAtom(popModalAtom);
  function showStickyAddDialog() {
    pushDialog(lazy(() => import("@/components/sticky-add-dialog"))).finally(
      () => popModal()
    );
  }

  const emitSearchCounter = useRef(0);
  const receivedSearchCounter = useRef(0);
  const [searchInput, setSearchInput] = useState("");
  const [page, setPage] = useState(0);

  const [stickyData, setStickyData] = useState<StickyThumb[]>([]);

  useEffect(() => {
    emitSearchCounter.current++;
    const taskId = emitSearchCounter.current;
    (async () => {
      if (taskId > receivedSearchCounter.current) {
        receivedSearchCounter.current = taskId;
        const sticky = await searchSticky(searchInput, page);
        if (sticky) {
          setStickyData(sticky);
        }else{
          console.log("empty?")
        }
      }
    })();
  }, [page, searchInput]);

  return (
    <div>
      <div className="flex items-center p-2 h-16 border-b space-x-1">
        <div className="relative ml-auto flex-1 md:grow-0">
          <LuSearch className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            type="search"
            placeholder="Search..."
            className="w-full rounded-lg bg-background pl-8 md:w-[200px] lg:w-[336px]"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
          />
        </div>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button>
              <LuMenu />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent>
            <DropdownMenuItem>
              <LuSearchSlash className="mr-2 h-4 w-4" />
              <span>Advanced Search</span>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuGroup>
              <DropdownMenuItem onClick={showStickyAddDialog}>
                <LuImagePlus className="mr-2 h-4 w-4" />
                <span>Add Sticky</span>
              </DropdownMenuItem>
              <DropdownMenuItem>
                <LuTextCursorInput className="mr-2 h-4 w-4" />
                <span>Add Text</span>
              </DropdownMenuItem>
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <StickyList stickies={stickyData} page={page} onPageChange={setPage}/>
    </div>
  );
}
