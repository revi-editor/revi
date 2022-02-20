-- mapper:nmap("g", {"yy", "p", "P"})
-- mapper:map("Insert", "<C-a>", "DeleteChar")

--This brought to my attention how limiting the current key engine is.
--No custom commands can be sent in "Insert" mode.
mapper:map(mode("Insert"), "<C-a>",{command("DeleteChar")})
--No over lapping commands can be made so if you bind something to `g` then `gg`
--will never be seen.
mapper:map(mode("Normal"), "g",{command("ScrollDown"), command("DeleteChar")})


