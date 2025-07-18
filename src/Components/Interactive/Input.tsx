import { cn } from "@/Miscellaneous/utility.ts"
import {cva, VariantProps} from "class-variance-authority";
import React from "react";

const inputVariants = cva(
    "border-2 focus:border-gray-700 focus:ring-tertiary file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-9 w-full min-w-0 rounded-md border text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
    {
        variants: {
            serial: {
                auth: "w-80 h-12 pl-12 pr-4 rounded-full",
                profile:"px-4 py-1 rounded-lg",
                search:"pr-10 min-w-96 pl-4 py-6 rounded-full"
            },
            color:{
                default: 'bg-white',
                cream: 'bg-[#F7F8F1]'
            },
        },
        defaultVariants: {
            serial: "profile",
            color: "default"
        },
    }
)

function Input({
                   serial,color,
                   ...props
               }: React.ComponentProps<"input"> &
    VariantProps<typeof inputVariants>) {
    return (
        <input
            className={cn(inputVariants({ serial, color}))}
            {...props}
        />
    )
}


export { Input, inputVariants }
