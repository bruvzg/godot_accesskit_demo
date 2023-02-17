#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/core/binder_common.hpp>

#include <godot_cpp/classes/global_constants.hpp>
#include <godot_cpp/classes/display_server.hpp>
#include <godot_cpp/classes/display_server_extension.hpp>
#include <godot_cpp/classes/display_server_extension_manager.hpp>

#include <godot_cpp/variant/utility_functions.hpp>

using namespace godot;

typedef void (*AccessKitAction)(void *, const char *);

// Functions exported by AccessKit library.
extern "C" void accesskit_init(void *p_dse, int64_t p_native_window_handle, AccessKitAction p_action, const char *p_update);
extern "C" void accesskit_destroy(void *p_dse, int64_t p_native_window_handle);
extern "C" void accesskit_push_update(void *p_dse, int64_t p_native_window_handle, const char *p_update);

// DisplayServer extension.
class AccessKitDSE : public DisplayServerExtension {
	GDCLASS(AccessKitDSE, DisplayServerExtension);

protected:
	static void _bind_methods() {
		ClassDB::bind_method(D_METHOD("update_tree", "id", "update"), &AccessKitDSE::update_tree);

		ADD_SIGNAL(MethodInfo("action_signal", PropertyInfo(Variant::STRING, "name")));
	}

public:
	// Callback function for AccessKit library.
	static void accesskit_action(void *p_dse, const char *p_name) {
		UtilityFunctions::print("[!] callback");
		((AccessKitDSE *)p_dse)->emit_signal("action_signal", String::utf8(p_name));
	}

	virtual String _get_name() const override {
		return String("AccessKit");
	}

	virtual void _create_window(int32_t p_window_id, int64_t p_native_window_handle) override {
		UtilityFunctions::print("[!] creare window");
		accesskit_init(this, p_native_window_handle, accesskit_action, "{\"nodes\":[{\"id\":1,\"role\":\"window\",\"name\":\"Hello from Godot\"}],\"focus\":1,\"tree\":{\"root\":1}}");
	}

	virtual void _destroy_window(int32_t p_window_id, int64_t p_native_window_handle) override {
		UtilityFunctions::print("[!] destroy window");
		accesskit_destroy(this, p_native_window_handle);
	}

	void update_tree(int32_t p_window_id, const String &p_update) {
		UtilityFunctions::print("[!] update");
		int64_t native_window_handle = DisplayServer::get_singleton()->window_get_native_handle(DisplayServer::WINDOW_HANDLE, p_window_id);
		accesskit_push_update(this, native_window_handle, p_update.utf8().get_data());
	}

	AccessKitDSE() {
		UtilityFunctions::print("[!] dse init");
	}
	~AccessKitDSE() {
		UtilityFunctions::print("[!] dse uninit");
	}
};

// GDExtension init.
void initialize_accesskit_module(ModuleInitializationLevel p_level) {
	if (p_level != MODULE_INITIALIZATION_LEVEL_SERVERS) {
		return;
	}

	UtilityFunctions::print("[!] module init");

	GDREGISTER_CLASS(AccessKitDSE);
	DisplayServerExtensionManager *dseman = DisplayServerExtensionManager::get_singleton();
	if (dseman) {
		Ref<AccessKitDSE> dse;
		dse.instantiate();
		dseman->add_interface(dse);
	}
}

void uninitialize_accesskit_module(ModuleInitializationLevel p_level) {
	if (p_level != MODULE_INITIALIZATION_LEVEL_SERVERS) {
		return;
	}

	UtilityFunctions::print("[!] module uninit");
}

extern "C" GDExtensionBool GDE_EXPORT accesskit_library_init(const GDExtensionInterface *p_interface, GDExtensionClassLibraryPtr p_library, GDExtensionInitialization *r_initialization) {
	godot::GDExtensionBinding::InitObject init_obj(p_interface, p_library, r_initialization);

	init_obj.register_initializer(initialize_accesskit_module);
	init_obj.register_terminator(uninitialize_accesskit_module);
	init_obj.set_minimum_library_initialization_level(MODULE_INITIALIZATION_LEVEL_SERVERS);

	return init_obj.init();
}
