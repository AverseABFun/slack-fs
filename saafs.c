/*
 * SaaFS - Slack as a FileSystem
 */

#include <linux/module.h> /* Needed by all modules */
#include <linux/printk.h> /* Needed for pr_info() */

int init(void)
{

	pr_info("SaaFS (init)\n");

	/* A non 0 return means init_module failed; module can't be loaded. */
	return 0;
}

void cleanup(void)
{
	pr_info("SaaFS (exit)\n");
}

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Arthur Beck <averse.abfun@gmail.com>");
MODULE_DESCRIPTION("Slack as a FileSystem");
MODULE_VERSION("1.0");
MODULE_ALIAS("sfs");
MODULE_ALIAS("slfs");
module_init(init);
module_exit(cleanup);