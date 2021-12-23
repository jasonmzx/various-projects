package net.fabricmc.rippedvillager;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;
import net.fabricmc.fabric.api.client.rendering.v1.EntityModelLayerRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.EntityRendererRegistry;
import net.minecraft.client.render.entity.model.EntityModelLayer;
import net.minecraft.util.Identifier;

@Environment(EnvType.CLIENT)
public class EntityTestingClient implements ClientModInitializer {
    public static final EntityModelLayer MODEL_CUBE_LAYER = new EntityModelLayer(new Identifier("entitytesting", "cube"), "main");
    @Override
    public void onInitializeClient() {
        /*
         * Registers our Cube Entity's renderer, which provides a model and texture for the entity.
         *
         * Entity Renderers can also manipulate the model before it renders based on entity context (EndermanEntityRenderer#render).
         */
        EntityRendererRegistry.register(RippedVillager.CUBE, (context) -> {
            return new RippedVillager.StrongVillagerRenderer(context);
        });
        // In 1.17, use EntityRendererRegistry.register (seen below) instead of EntityRendererRegistry.INSTANCE.register (seen above)
        EntityRendererRegistry.register(RippedVillager.CUBE, (context) -> {
            return new RippedVillager.StrongVillagerRenderer(context);
        });
 
        EntityModelLayerRegistry.registerModelLayer(MODEL_CUBE_LAYER, RippedVillager.StrongVillagerModel::getTexturedModelData);
    }
}