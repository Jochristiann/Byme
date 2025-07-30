import { useState, useRef, useEffect } from "react";
import { Button } from "./Button";
import {FaAngleDown} from "react-icons/fa";

interface DropdownItem {
    label: string;
    value: string;
}

interface DropdownProps {
    items: DropdownItem[];
    selected: string;
    onSelect: (value: string) => void;
    placeholder?: string;
}

export default function Dropdown({
                                     items,
                                     selected,
                                     onSelect,
                                     placeholder = "Select an option",
                                 }: DropdownProps) {
    const [open, setOpen] = useState(false);
    const dropdownRef = useRef<HTMLDivElement>(null);

    // Close dropdown on outside click
    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (
                dropdownRef.current &&
                !dropdownRef.current.contains(event.target as Node)
            ) {
                setOpen(false);
            }
        };
        document.addEventListener("mousedown", handleClickOutside);
        return () => document.removeEventListener("mousedown", handleClickOutside);
    }, []);

    return (
        <div className="relative inline-block w-64" ref={dropdownRef}>
            <Button
                onClick={() => setOpen((prev) => !prev)}
                className="w-full border border-gray-300 bg-white px-4 py-2 rounded-xl shadow-sm text-left text-sm text-gray-700 hover:bg-gray-50 flex flex-row justify-between items-center"
            >
                {selected || placeholder}
                <FaAngleDown/>
            </Button>

            {open && (
                <ul className="absolute z-10 mt-1 w-full bg-white border border-gray-200 rounded-xl shadow-lg max-h-60 overflow-auto text-sm">
                    {items.map((item) => (
                        <li
                            key={item.value}
                            onClick={() => {
                                onSelect(item.value);
                                setOpen(false);
                            }}
                            className="px-4 py-2 cursor-pointer hover:bg-gray-100"
                        >
                            {item.label}
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
}
