<?php
srand(1234);
echo "srand(1234);\n";
for($x=0;$x<40;++$x)
{
	echo rand(0,100).", ";
}
echo "\n";

exit;
