#include <stdio.h>
#include <libscf.h>

int
main(int argc, char **argv)
{
	char buf[512];
	ssize_t len;
	scf_handle_t *h = scf_handle_create(SCF_VERSION);
	if (scf_handle_bind(h) != 0) {
		(void) fprintf(stderr, "Bind failed\n");
		return (1);
	}
	len = scf_myname(h, buf, sizeof (buf));
	if (len < 0) {
		(void) fprintf(stderr, "Failed to get service name\n");
		return (1);
	}
	(void) printf("%s\n", buf);
	scf_handle_destroy(h);
	return (0);
}
